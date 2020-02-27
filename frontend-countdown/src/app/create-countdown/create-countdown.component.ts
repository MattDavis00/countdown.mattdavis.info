import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-create-countdown',
  templateUrl: './create-countdown.component.html',
  styleUrls: ['./create-countdown.component.css']
})
export class CreateCountdownComponent implements OnInit {

  theme: boolean = false;
  year: string = "2020";
  month: string = "12";
  day: string = "31";
  hour: string = "16";
  minute: string = "30";

  constructor() { }

  ngOnInit() {
    var currentDate = new Date();
    this.year = currentDate.getFullYear().toString();
    this.month = (currentDate.getMonth() + 1).toString();
    this.day = currentDate.getDate().toString();
    this.hour = currentDate.getHours().toString();
    this.minute = currentDate.getMinutes().toString();
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
