import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import backendURL from '../backendURL';
import { Router } from '@angular/router';

@Component({
  selector: 'app-create-countdown',
  templateUrl: './create-countdown.component.html',
  styleUrls: ['./create-countdown.component.css']
})
export class CreateCountdownComponent implements OnInit {

  theme: boolean = false;
  title: string = "";
  year: string = "2020";
  month: string = "12";
  day: string = "31";
  hour: string = "16";
  minute: string = "30";

  constructor(private http: HttpClient, public router: Router) { }

  ngOnInit() {
    var currentDate = new Date();
    this.year = currentDate.getFullYear().toString();
    this.month = (currentDate.getMonth() + 1).toString();
    this.day = currentDate.getDate().toString();
    this.hour = currentDate.getHours().toString();
    this.minute = currentDate.getMinutes().toString();
  }

  create() {
    var timeToSet = new Date(parseInt(this.year), parseInt(this.month) - 1, parseInt(this.day), parseInt(this.hour), parseInt(this.minute), 0, 0);
    var utcString = timeToSet.toUTCString();

    this.http.post<{
      success: boolean;
      errors: [];
      id: string;
      name: string;
      datetime: string;
    }>(backendURL + "/create",
    {
      "name": {"data": this.title, "id": "title"},
      "datetime": {"data": utcString, "id": "datetime"}
    })
    .subscribe(
      data  => {
        console.log("POST Request is successful ", data);
        if (data.success) {
          this.router.navigate(['/id/' + data.id]);
        } else {
          // this.errService.handleErrors(this.el, data.errors, ["email", "password"]);
        }
      },
      error  => {
        console.log("Error", error);
        // this.errService.handleErrors(this.el, [{"id": "fatal", "reason": "Unable to perform request. Please try again."}], []);
      }

    );
  }

  themeChange() {
    if (this.theme) {
      document.getElementsByClassName('column')[0].classList.remove("dark");
      document.getElementsByClassName('clock')[0].classList.remove("dark");
    } else {
      document.getElementsByClassName('column')[0].classList.add("dark");
      document.getElementsByClassName('clock')[0].classList.add("dark");
    }
  }

}
